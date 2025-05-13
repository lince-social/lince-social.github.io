# Lince

## Tool for registry, interconnection and automation of Needs and Contributions with open scope.

@: What is Lince?

%: Lince is a tool for registry, interconnection and automation of Needs and Contributions with open scope.

@: Ok, but what is it?

%: Lince is an app that can be used to model and/or automate personal tasks, items, computer actions, economic trades between parties... The limit is your imagination (and your wizard skills with computers).

@: Sure...

%: Look, I'll explain everything in detail, so follow me. I promise the journey is worth the end, traveler.

Lince works with a PostgreSQL database, some answers about data types can be found there, it searches for it's data in ~/.config/lince/lince.sql, if not found there, it defaults to the directory Lince was cloned to, in src/db/versions/lince.sql. It is recomended to frequently backup the lince.sql file, if some error or mistake happens, your information is safe.

It is tempting to say TCITD, or 'The code is the documentation', because documentations are often not up to date and that is the best way of seeing the truth about the program. But that is not best for everyone, so this documentation in text was made. With that said, if you want to learn more about lince by reading the documentation, it is advised to also have the database declaration open: https://github.com/lince-social/lince/blob/main/src/db/schema.sql, for the documentation below has the name and data type of the tables, but not it's constraints.

Firstly the tables of the database will be explained, then the ways in which they can interact with themselves and/or your/other computers:
