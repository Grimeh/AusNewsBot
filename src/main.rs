// Content written by Andy Astruc (twitter: meltedmasks) and Stef Peacock (twitter: Stef Peacock)
// Coded by Brandon Grimshaw (gh: Grimeh, twitter: BrandonGrimshaw)

use rand::prelude::*;

// TODO
//  - add capability to specify per-word joiners
//      - eg. a "sleeping" verb could have "with" suffixed when used in a template where the verb requirement isn't the last word
//      - "You won't believe what millenials are doing to save money: sleeping with spiders"

// # HOW TO WRITE TEMPLATES
// ## FORMAT
// HINT: `[]` indicates that something is optional
// {value[:case]}
//
// ## EXAMPLES
//  `{t}` or `{t:_}` or `{t:p}`
//      Topic with no case manipulation.
//      `Dungeons & Dragons` => `Dungeons & Dragons`
//
//  `{v:u}`
//      Verb with all letters made uppercase.
//      `huffing` => `HUFFING`
//
//  `{demo:c}`
//      Demographic with first letter capitalised.
//      `millennials` => `Millennials`
//
// ## VALUES
// t: topic
// f: flavour
// v: verb
// demo: demographic
//
// ## CASES
// p: passthrough, don't change (DEFAULT)
// c: capitalise, make first letter uppercase
// u: uppercase, make all letters uppercase
// l: lowercase, make all letters lowercase

// "BLANK? Meet the BLANKS who want to BLANK your kids",
// "Behind the BLANK: This new BLANK makes BLANK BLANK BLANK",
const TEMPLATES: &[&str] = &[
	"{t:c}: Sinister gateway to {t}?!",
	"Young people are getting into {t}, but it hides a dark secret: {f} {t}",
	"Children are stuck on the latest dangerous craze: {v:u} {t:u}",
	"You won't believe what {demo} are doing to save money: {v} {t}"
];

const FLAVOUR: &[&str] = &[
	"murder",
];

const TOPICS: &[&str] = &[
	"drugs",
	"kittens",
	"death",
	"Pokemon",
	"Dungeons & Dragons",
	"filth",
	"spiders",
];

const VERBS: &[&str] = &[
	"huffing",
	"eating",
	"licking",
	"mainlining",
	"injecting",
	"hoarding",
	"snorting",
	"killing",
	"kissing",
	"winning",
	"slinging",
	"sleeping",
	"swinging",
];

const DEMOGRAPHICS: &[&str] = &[
	"children",
	"teenagers",
	"millenials",
];

enum RequirementType {
	Noun,
	Flavour,
	Verb,
	Demographic,
}

impl RequirementType {
	pub fn parse(t: &str) -> RequirementType {
		use RequirementType::*;
		match t {
			"t" => RequirementType::Noun,
			"f" => RequirementType::Flavour,
			"v" => RequirementType::Verb,
			"demo" => RequirementType::Demographic,
			_ => panic!("unrecognised token {t}")
		}
	}

}

enum Case {
	/// Passthrough, don't change,  "toPiC" => "toPiC"
	Pass,

	/// Capitalise first letter,    "toPiC" => "ToPiC"
	Capitalise,

	/// Make ALL letters uppercase, "toPiC" => "TOPIC"
	Upper,

	/// Make ALL letters lowercase, "toPiC" => "topic"
	Lower,
}

impl Case {
	pub fn parse(s: &str) -> Case {
		use Case::*;
		match s {
			"_" | "p" => Pass,
			"c" => Capitalise,
			"u" => Upper,
			"l" => Lower,
			_ => panic!("unrecognised case {}", s)
		}
	}

	pub fn transform_str(&self, input: &str) -> String {
		let mut result = input.to_string();

		match self {
			Case::Pass => {}
			Case::Capitalise => {
				if let Some(c) = result.get_mut(0..1) {
					c.make_ascii_uppercase();
				}
			}
			Case::Upper => {
				result = result.to_uppercase();
			}
			Case::Lower => {
				result = result.to_lowercase();
			}
			_ => unimplemented!()
		}

		result
	}
}

struct Requirement {
	ty: RequirementType,
	case: Case,
}

impl Requirement {
	fn parse(token: &str) -> Requirement {
		// TODO split token
		let subtokens: Vec<_> = token.split_terminator(':').collect();

		assert!(subtokens.len() > 0); // must have a value

		let ty = RequirementType::parse(subtokens[0]);

		let mut case = Case::Pass;
		if let Some(c) = subtokens.get(1) {
			case = Case::parse(c);
		}

		Requirement {
			ty,
			case,
		}
	}
}

fn generate(rng: &mut ThreadRng) -> String {
	// choose template
	let template = TEMPLATES[rng.gen_range(0..TEMPLATES.len())];
	let mut result = String::with_capacity(template.len() * 2);

	// extract required inputs
	let mut idx = 0;
	while idx < template.len() {
		let section = &template[idx..];
		let start = section.find('{').and_then(|i| Some(i + idx));
		let end = section.find('}').and_then(|i| Some(i + idx));

		if start.is_none() || end.is_none() {
			result.push_str(section);
			break;
		}

		if start > end {
			panic!("read }} before {{");
		}

		let start = start.unwrap();
		let end = end.unwrap();

		let token = &template[(start + 1)..end];
		let requirement = Requirement::parse(token);

		let replacement = match requirement.ty {
			RequirementType::Noun => TOPICS[rng.gen_range(0..TOPICS.len())],
			RequirementType::Flavour => FLAVOUR[rng.gen_range(0..FLAVOUR.len())],
			RequirementType::Verb => VERBS[rng.gen_range(0..VERBS.len())],
			RequirementType::Demographic => DEMOGRAPHICS[rng.gen_range(0..DEMOGRAPHICS.len())],
			_ => unimplemented!()
		}.to_string();
		let replacement = requirement.case.transform_str(&replacement);

		result.push_str(&template[idx..start]);
		result.push_str(&replacement);

		idx = end + 1;
	}

	result
}

const ITERATIONS: usize = 5000;

fn main() {
	let mut rng = rand::thread_rng();
	let mut results = Vec::with_capacity(ITERATIONS);

	for _ in 0..ITERATIONS {
		let x = generate(&mut rng);
		if !results.contains(&x) {
			results.push(x);
		}
	}

	std::fs::write("./output.txt", &results.join("\n")).unwrap();
}
