use bot::allocators::allocator::IAllocator;
use bot::bot::bot::IBot;

pub struct BotConfig<A: IAllocator, B: IBot> {
    pub allocator: A, 
    pub bot: B, 
}

impl<A: IAllocator, B: IBot> BotConfig<A, B> {
    pub fn new(allocator: A, bot: B) -> Self {
        Self { allocator, bot }
    }
}