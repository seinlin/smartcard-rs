// @Author: ronan
// @Date:   21-12-2016
// @Email:  ronan.lashermes@inria.fr
// @Last modified by:   ronan
// @Last modified time: 21-12-2016

use safe::context::Context;

///The card reader is what is really plugged into the computer.
pub struct Reader {
    name: String,
    context: Context
}

impl Reader {

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
