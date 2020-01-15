use fake::{Dummy, Fake, Faker};
use juniper::FieldResult;
use juniper::RootNode;

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
    for _ in 0..=20 {
        let mut auth = Faker.fake::<Author>();
        for _ in 0..4 {
            auth.books.push(Faker.fake::<Books>());
        }
        result.push(auth);
    }
    result
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field authors(&executor) -> FieldResult<Vec<Author>> {
        Ok(gen_data())
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
