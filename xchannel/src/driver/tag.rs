use crate::tag::vtype::*;

use crate::tag::Tag as MTag;

pub struct Tag {
    pub name: String,
    pub value: Value,
    pub dtype: DataType,
    pub address: String,
}

impl From<MTag> for Tag {
	fn from(tag: MTag) -> Self {
		Tag {
			name: tag.name,
			value: tag.value,
			dtype: tag.dtype,
			address: tag.address.unwrap(),
		}
	}
}