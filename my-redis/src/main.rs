use tokio::{
    io::{AsyncBufReadExt, BufReader, AsyncWriteExt},
    net::TcpListener,
    sync::broadcast,
};

fn give_me_default<T>() -> T 
where 
    T: Default,
{ Default::default() }

#[tokio::main]
async fn main() {

    let value = give_me_default::<i32>();
    // Bind the listener to the address
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // Starts a broadcast channel
    let (tx, mut _rx) = broadcast::channel(10);

    loop {

        // Socket + address
        let(mut socket, addr) = listener.accept().await.unwrap();


        // We get rid of the error of moving a value into a loop
        // by cloning tx.
        let tx = tx.clone();

        // Gets a receiver from the sender
        let mut rx = tx.subscribe();


        // Creates new task so multiple 'tasks' can be run at the same time
        // Although threads are concurrent, tasks are not. That is why we
        // do this.

        // It moves all client handling into a new task
        
        tokio::spawn(async move /*async block*/ {

            // Reading + writing a line
            let(reader, mut writer) = socket.split();

            // Mutatable and uses tokio function
            let mut reader = BufReader::new(reader);

            // The line is passed as a string
            let mut line = String::new(); 

            // let mut strSock = reader.to_string();
            let mix = String::from("User from server: ");

            loop {

                // Acts on multiple concurrent events at the same time
                // and returns the first one that comes back.
                tokio::select! {
                    result = reader.read_line(&mut line) => {

                        // Same as bytes read below
                        if result.unwrap() == 0{
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();

                        // Clears the line, otherwise it would be constantly sending
                        // every single message
                        line.clear();  

                    }
                    result = rx.recv() => { // Returns a future

                        let (msg, other_addr) = result.unwrap(); // Unwraps the future
                        
                        if addr!= other_addr{

                            writer.write_all(mix.as_bytes()).await.unwrap();
                            writer.write_all(msg.as_bytes()).await.unwrap();

                        }
                    }
                }
                
                // The bytes read comes from the line
                /*let bytes_read = reader.read_line(&mut line).await.unwrap();
                
                // If it does not detect a connection it exits
                if bytes_read == 0{
                    break;
                }

                tx.send(line.clone()).unwrap(); // Has to be unwrapped as it contains a result

                let msg = rx.recv().await.unwrap();

                // Writes the line 
                */
            }
        });
    }
}
