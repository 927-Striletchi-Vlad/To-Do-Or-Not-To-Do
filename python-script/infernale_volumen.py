from faker import Faker
from tqdm import tqdm

fake = Faker()

f = open("up1.sql", "w")


# Generate 1_000_000 user ids
print("Generating 1_000_000 user ids")
user_ids = [fake.uuid4() for _ in range(1_000_000)]
print("Generating 1_000_000 todo ids")
todo_ids = [fake.uuid4() for _ in range(1_000_000)]
print("Generating 1_000_000 todolist ids")
todolist_ids = [fake.uuid4() for _ in range(1_000_000)]
time = "2023-04-04 00:00:00"

# Generate 1_000_000 users
print("Generating 1_000_000 users")
for i in tqdm(range(10000)):
    name = fake.name()
    email = fake.email()
    password = fake.password()
    temp = ""
    temp += "INSERT INTO users (uid, name, email, password, created_at) VALUES  \n "

    for j in range(100):
        temp += (
            "('"
            + user_ids[i * 100 + j]
            + "', '"
            + name
            + str(j)
            + "', '"
            + email
            + str(j)
            + "', '"
            + password
            + str(j)
            + "', '"
            + time
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)

f.close()
f = open("up2.sql", "w")

# Generate 1_000_000 todos
print("Generating 1_000_000 todos")
for i in tqdm(range(10000)):
    title = fake.sentence()
    content = fake.text()
    completed = str(fake.boolean())
    temp = ""
    temp += "INSERT INTO todos (tid, title, content, completed, created_at) VALUES  \n "
    for j in range(100):
        temp += (
            "('"
            + todo_ids[i * 100 + j]
            + "', '"
            + title
            + str(j)
            + "', '"
            + content
            + str(j)
            + "', '"
            + completed
            + "', '"
            + time
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)

f.close()
f = open("up3.sql", "w")

# Generate 1_000_000 todolists
print("Generating 1_000_000 todolists")
for i in tqdm(range(10000)):
    title = fake.sentence()
    priority = str(fake.random_int(min=1, max=9))
    temp = ""
    temp += "INSERT INTO todolists (tlid, uid, title, priority, created_at) VALUES  \n "
    for j in range(100):
        temp += (
            "('"
            + todolist_ids[i * 100 + j]
            + "', '"
            + user_ids[i * 100 + j]
            + "', '"
            + title
            + str(j)
            + "', '"
            + priority
            + "', '"
            + time
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)

f.close()
f = open("up4.sql", "w")

# Generate 10_000_000 todostodolists
# Put 10 todos in each todolist
print("Generating 10_000_000 todostodolists")
for i in tqdm(range(10000)):
    temp = ""
    temp += "INSERT INTO todostodolists (tid, tlid) VALUES  \n "
    for j in range(100):
        for k in range(10):
            temp += (
                "('"
                + todo_ids[(i * 100 + j + k) % 1_000_000]
                + "', '"
                + todolist_ids[i * 100 + j]
                + "'),\n"
            )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)

print("Done")
f.close()
