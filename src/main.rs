extern crate clang;

use clang::{Clang, Index, Entity, EntityKind};

fn into_sources(entity: Entity)
    -> String
{
    entity.get_range().unwrap()
        .tokenize()
        .into_iter()
        .map(|token| token.get_spelling())
        .fold(String::new(), |previous, token| format!("{} {}", previous, token))
}

fn parse_list(init_list: Entity)
{
    println!("parsing list recursively ...");

    let sources = into_sources(init_list);
    println!("== {}", sources);

    let assignments = init_list
        .get_children();

    for field in assignments
    {
        let name = field.get_child(0).unwrap().get_display_name().unwrap();
        let value = into_sources(field.get_child(1).unwrap());

        let info = if name != "pf_foo" { String::from("") } else {
            let func_type = field
                .get_child(1)
                .unwrap()
                .get_child(0)
                .unwrap()
                .get_definition();

            format!(" == {}", func_type.unwrap().get_display_name().unwrap())
        };

        println!(".{member_name} = {value} {info}",
                 member_name = name,
                 value = value,
                 info = info);
    }
}

fn display_recursively(entity: Entity, level: usize)
{
    println!("{:indent$}{entity:?}", "", indent=level*4, entity=entity);
    for field in entity.get_children()
    {
        display_recursively(field, level + 1)
    }
}

fn main()
{
    let clang = Clang::new()
        .expect("Can't create clang instance");

    let index = Index::new(&clang, false, false);

    let tu = index.parser("examples/test.c")
        .parse()
        .expect("Can't parse translation unit");

    let structs = tu.get_entity()
        .get_children()
        .into_iter()
        //.filter(|e| e.get_kind() == EntityKind::VarDecl)
        .collect::<Vec<_>>();

    for struct_decl in structs
    {
        let struct_type = struct_decl
            .get_type()
            .unwrap();

        if struct_type.get_display_name() != "struct test_struct"
        {
        }

        println!("analysing struct variable declaration {} of type {}",
                 struct_decl.get_name().unwrap(),
                 struct_type.get_display_name());

        display_recursively(struct_decl, 0);

        for field in struct_decl.get_children()
        {
            let name = field.get_display_name();
            println!("- analysing field {:?}", field);
            match field.get_kind()
            {
                EntityKind::InitListExpr => parse_list(field),
                _ => println!("Unhandled EntityKind type"),
            }
        }
    }
}
