use fake::{Dummy, Fake, Faker};
use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject, Debug, Dummy)]
#[graphql(description = "An author")]
struct Author {
    id: String,
    name: String,
    company: String,
    books: Vec<Books>,
}

#[derive(GraphQLObject, Debug, Dummy)]
#[graphql(description = "An author")]
struct Books {
    id: String,
    name: String,
    num_pages: i32,
}

fn gen_data() -> Vec<Author> {
    let mut result: Vec<Author> = Vec::new();
    for _ in 0..21 {
        let mut auth = Faker.fake::<Author>();
        for _ in 0..4 {
            auth.books.push(Faker.fake::<Books>());
        }
        result.push(auth);
    }
    result
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    },
    field authors(&executor) -> FieldResult<Vec<Author>> {
        Ok(gen_data())
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
