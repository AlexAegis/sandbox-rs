const NTH: [&str; 12] = [
	"first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
	"eleventh", "twelfth",
];

/// Austin, 1909
const ITEMS: [&str; 12] = [
	"A Partridge in a pear-tree",
	"Two turtle doves",
	"Three french hens",
	"Four colly birds",
	"Five gold rings",
	"Six geese a-laying",
	"Seven swans a-swimming",
	"Eight maids a-milking",
	"Nine ladies dancing",
	"Ten lords a-leaping",
	"Eleven pipers piping",
	"Twelve drummers drumming",
];

const SEPARATOR: &str = ",\n";
const LAST_SEPARATOR: &str = ", and\n";

fn nth_intro(n: usize) -> String {
	format!(
		"On the {} day of Christmas\nMy true love sent to me\n",
		NTH[n]
	)
}

pub fn christmas() -> String {
	(0..=11)
		.map(|i| {
			let mut verse = nth_intro(i);
			verse += &((0..(i))
				.map(|l| ITEMS[l])
				.collect::<Vec<&str>>()
				.join(SEPARATOR));
			verse += if i == 0 { "" } else { LAST_SEPARATOR };
			verse += ITEMS[i];
			verse
		})
		.collect::<Vec<String>>()
		.join("\n\n")
}

#[cfg(test)]
mod tests {

	use super::*;
	#[test]
	fn christmas_test() {
		assert_eq!(
			christmas(),
			"\
			 On the first day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree\n\
			 \n\
			 On the second day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree, and\n\
			 Two turtle doves\n\
			 \n\
			 On the third day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves, and\n\
			 Three french hens\n\
			 \n\
			 On the fourth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens, and\n\
			 Four colly birds\n\
			 \n\
			 On the fifth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds, and\n\
			 Five gold rings\n\
			 \n\
			 On the sixth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings, and\n\
			 Six geese a-laying\n\
			 \n\
			 On the seventh day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying, and\n\
			 Seven swans a-swimming\n\
			 \n\
			 On the eighth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying,\n\
			 Seven swans a-swimming, and\n\
			 Eight maids a-milking\n\
			 \n\
			 On the ninth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying,\n\
			 Seven swans a-swimming,\n\
			 Eight maids a-milking, and\n\
			 Nine ladies dancing\n\
			 \n\
			 On the tenth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying,\n\
			 Seven swans a-swimming,\n\
			 Eight maids a-milking,\n\
			 Nine ladies dancing, and\n\
			 Ten lords a-leaping\n\
			 \n\
			 On the eleventh day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying,\n\
			 Seven swans a-swimming,\n\
			 Eight maids a-milking,\n\
			 Nine ladies dancing,\n\
			 Ten lords a-leaping, and\n\
			 Eleven pipers piping\n\
			 \n\
			 On the twelfth day of Christmas\n\
			 My true love sent to me\n\
			 A Partridge in a pear-tree,\n\
			 Two turtle doves,\n\
			 Three french hens,\n\
			 Four colly birds,\n\
			 Five gold rings,\n\
			 Six geese a-laying,\n\
			 Seven swans a-swimming,\n\
			 Eight maids a-milking,\n\
			 Nine ladies dancing,\n\
			 Ten lords a-leaping,\n\
			 Eleven pipers piping, and\n\
			 Twelve drummers drumming"
		);
	}
}
