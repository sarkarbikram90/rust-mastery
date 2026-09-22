use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Channels are typed. We're making a channel that sends u32 values.
    // The first half of the tuple is the Sender (tx) and the second half is
    // the Receiver (rx).
    let (tx, mut rx) = mpsc::channel::<u32>(10);

    tokio::spawn(async move {
        // Send 0, 1, 2 in order
        tx.send(0).await.unwrap();
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
    });

    // The receive order depends on how the runtime schedules the tasks,
    // but usually they come out in the order they were sent.
    assert_eq!(rx.recv().await.unwrap(), 0);
    assert_eq!(rx.recv().await.unwrap(), 1);
    assert_eq!(rx.recv().await.unwrap(), 2);
}
