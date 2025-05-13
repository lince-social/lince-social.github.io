# Record

| record   |
| -------- |
| id       |
| quantity |
| head     |
| body     |
| location |

Lince is centered on the 'record' table, but like, according to the creator... like, what do they know?

Let's assume this is Lince's sun, it is the most capable one on the task of giving Lince life. The thing the app revolves around.

---

| record   | DATA STRUCTURE |
| -------- | -------------- |
| id       | SERIAL         |
| quantity | REAL           |
| head     | TEXT           |
| body     | TEXT           |
| location | POINT          |

'id's are automatically generated.

'quantity' represents the availability of the record, if negative it is a Necessity, if positive, a Contribution, zero makes it not mean much, sometimes.

'head' and 'body' are meant to be parts of a whole, where one can be used for a short summary and the other a description, or one has all the information and the other holds tags for filtering through views. With a pubsub protocol, one can send a short information of the record, in this case it can be the head, and put the rest in the body. Only those interested in the head will ask for the body of the record. That way the minimum amount of information is sent over the network, making it faster and stuff, I think.

'location' is an important information for interactions outside of computers (they exist, it's insane) or any other use you want to give it.

---

| record   | DATA STRUCTURE | USER INPUT |
| -------- | -------------- | ---------- |
| id       | SERIAL         |            |
| quantity | REAL           | -1         |
| head     | TEXT           | Eat Apple  |
| body     | TEXT           |            |
| location | POINT          |            |

So, for an example, imagine that you like apples and you want to create a task to eat it today.

You create a 'record', giving it '-1' to the 'quantity', for that action is a Necessity in your life right now, and 'Eat Apple' to the 'head'.

---

| record   | DATA STRUCTURE | USER INPUT | ACTUAL RECORD |
| -------- | -------------- | ---------- | ------------- |
| id       | SERIAL         |            | 1             |
| quantity | REAL           | -1         | -1            |
| head     | TEXT           | Eat Apple  | Eat Apple     |
| body     | TEXT           |            | NULL          |
| location | POINT          |            | NULL          |

The end result, on the PostgreSQL database, is this record. In summary, fields with 'NOT NULL' that have defaults don't need to be filled, as it happens automatically.

---

| id  | quantity | head      | body | location |
| --- | -------- | --------- | ---- | -------- |
| 1   | -1       | Eat Apple | NULL | NULL     |

The theoretical apple eater in the example chose to put -1 in 'quantity' because they have a view that gives them all the records with a negative 'quantity' (quantity < 0).

So they will see the 'Eat Apple' task on that view, but more about that in a second, now look at other examples of records (rows are set horizontally now).

---

| id  | quantity | head        | body                 | location |
| --- | -------- | ----------- | -------------------- | -------- |
| 1   | -1       | Eat Apple   | NULL                 | NULL     |
| 2   | 1        | Apple       | Item, Food           | NULL     |
| 3   | 0        | Brush Teeth | Action, Hygiene      | NULL     |
| 4   | 3        | Toothbrush  | Item, Hygiene        | NULL     |
| 5   | -1       | Meditate    | Action, Spirituality | NULL     |

As you can see, there are records with different quantities, heads, and bodies. They are modeling actions and items.

The user likes to center it's filtering through the body column, seeing all actions, or all items of a certain area of their lives, like Hygiene, each in different created views.
