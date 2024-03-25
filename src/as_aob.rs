pub trait AsAOB {
    fn size(&self) -> usize;
    fn as_aob_le(&self) -> Vec<u8>;
    fn as_aob_be(&self) -> Vec<u8>;
}

impl AsAOB for u8 {
    fn size(&self) -> usize {
        1
    }

    fn as_aob_le(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn as_aob_be(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsAOB for u16 {
    fn size(&self) -> usize {
        2
    }

    fn as_aob_le(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn as_aob_be(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsAOB for u32 {
    fn size(&self) -> usize {
        4
    }

    fn as_aob_le(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn as_aob_be(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsAOB for u64 {
    fn size(&self) -> usize {
        8
    }

    fn as_aob_le(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn as_aob_be(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
