pub struct OutputHandler {}
impl OutputHandler {
    pub(crate) async fn handle(receiver: tokio::sync::mpsc::Receiver<OutputFormat>) -> Result<()> {
        Ok(())
    }
}
pub struct OutputFormat {}
