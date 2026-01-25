pub mod app;

use bot::bot::bot::IBot;
pub enum PlayerType {
    Human, 
    Bot(Box<dyn IBot>),
}
