#[derive(Debug)]
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

#[derive(Debug)]
pub struct Block<BlockNumber, Extrinsic> {
    pub header: Header<BlockNumber>,
    pub extrinsics: Vec<Extrinsic>,
}

#[derive(Debug)]
pub struct Extrinsic<AccountId, Call> {
    pub caller: AccountId,
    pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
    type Caller;
    type Call;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}

impl<BlockNumber, Extrinsic> Block<BlockNumber, Extrinsic>
where
    BlockNumber: Copy + Default + PartialEq,
{
    pub fn new(block_number: BlockNumber, extrinsics: Vec<Extrinsic>) -> Self {
        Block {
            header: Header { block_number },
            extrinsics,
        }
    }
}
