template = 'let {identifier} = PropertyCard::new(String::from("{prop_name}"), Color::{color});'

cards = {
    'Brown': ['Baltic Avenue', 'Mediterranean Avenue'],
    'Blue': ['Broadwalk', 'Park Place'],
    'Green': ['North Carolina Avenue', 'Pacific Avenue', 'Pennsylvania Avenue'],
    'LightBlue': ['Connecticut Avenue', 'Oriental Avenue', 'Vermont Avenue'],
    'Orange': ['New York Avenue', 'St. James Place', 'Tennesse Avenue'],
    'Pink': ['St. Charles Place', 'Virginia Avenue', 'States Avenue'],
    'Black': ['Short Line', 'B. & O. Railroad', 'Reading Railroad', 'Pennsylvania Railroad'],
    'Red': ['Kentucky Avenue', 'Indiana Avenue', 'Illinois'],
    'LightGreen': ['Water Works', 'Electric Company'],
    'Yellow': ['Ventor Avenue', 'Marvin Gardens', 'Atlantic Avenue'],
}

for color, prop_names in cards.items():
    for prop_name in prop_names:
        print(template.format(prop_name=prop_name, color=color.capitalize(),
            identifier=prop_name.lower().replace('.', '').replace(' ', '_')))
    print()
