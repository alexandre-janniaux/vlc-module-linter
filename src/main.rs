extern crate clang;

use clang::{Clang, Index, Entity, EntityKind};

fn parse_list(init_list: Entity)
{
    println!("parsing list recursively ...\n{:?}", init_list);
    let sources = init_list.get_range().unwrap()
        .tokenize()
        .into_iter()
        .map(|token| token.get_spelling())
        .fold(String::new(), |previous, token| format!("{} {}", previous, token));

    println!("== {}", sources);

    for field in init_list.get_children()
    {
        println!("==> {:?}\n", field);
        parse_list(field);
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

        println!("analysing evaluation: {:?}", struct_decl.evaluate());
        println!("analysing definition: {:?}", struct_decl.get_definition().unwrap().evaluate());

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
