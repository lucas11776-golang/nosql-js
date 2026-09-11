Must have type infer

```typescript

class Collection {
    find<T>(filter: any): T[];
}

class Database {
    static open(path: string): Promise<Database> {
        // Some Code.
    }

    collection(name: string): Promise<Collection> {
        // Some Code.
    }
}

interface User {
    _id: string,
    email: string
}

async function main() {
    let db = aw Database.open("db.bin");

    let users = await db.collection("users");

    let user = await users.find<User>({email: "thembangubeni04@gmail.comn"});

    console.log(`User Record: ${JSON.stringify(guest)}`);
}

main()
    .then()
    .catch((err:unknown) => {
        console.log(`Error: ${err}`);
    });
```