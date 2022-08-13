//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::model::types::{ThreadBox, ThreadBoxType};
use base::hex_utils::hex_string;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for ThreadBox {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Type: {}. ", self.thread_box_type)?;
        writeln!(f, "Thread ids: {}. ", self.thread_ids.len())?;
        self.thread_ids.iter().for_each(|id| {
            let _ = writeln!(f, "  Id: {} ", hex_string(id));
        });

        Ok(())
    }
}

impl ThreadBox {
    pub fn new(thread_box_type: ThreadBoxType) -> ThreadBox {
        ThreadBox {
            thread_box_type: thread_box_type as i32,
            thread_ids: vec![],
        }
    }

    /// returns true iff threadbox has thread_id
    pub fn has_thread(&self, thread_id: &[u8]) -> bool {
        self.thread_ids.iter().find(|id| *id == thread_id).is_some()
    }
}
