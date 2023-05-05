use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::TcpListener, sync::broadcast,
};


#[tokio::main] 
async fn main() {
    // listen for incoming connections, 
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // to have multiple producers that can
    // send messages to the channel and  multiple consumer that can receive messages from the channel
    let (tx, rx) = broadcast::channel(10); 
    
    loop {
       
        //accept incoming connections
        let (mut socket, addr) = listener.accept().await.unwrap();


        let tx = tx.clone(); 
    

        let mut rx = tx.subscribe();  
        
        // handle multiple connections concurrently
        tokio::spawn(async move { 

        //  use the reader and writer independently
        let (reader, mut writer) = socket.split(); 

       

        //  buffRead method on the socket to read the data from the socket
        let mut reader = tokio::io::BufReader::new(reader);

        let mut line = String::new();

        loop {

            // select between multiple futures, 
            tokio::select! {
                //read a line from the socket and store it in the line variable
               
                result = reader.read_line(&mut line) => {
                    if result.unwrap() == 0 {
                        break;
                    }
                    //  send the data to all the receivers
                    // we are also sending the address of the client, so that we can differentiate between the clients
                    tx.send((line.clone(), addr)).unwrap();
                    
                    line.clear();
                }

                // here result is the message that is received from the channel
                // we will use the write_all method to write the message to the socket,
                result = rx.recv() => {
                    let (msg, other_addr) = result.unwrap(); 
                    if addr != other_addr { // 
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }

        }
    });
    }
}

