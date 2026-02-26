#let by(start: bool, name: str, email: str, date: datetime) = [
  #align(if start { left } else { right + bottom })[
    #name | #email

    #date.display()
  ]
]

#let post(name: str, email: str, date: datetime, body) = [
  #let this_by(start: bool) = by(
    start: start,
    name: name,
    email: email,
    date: date,
  )

  #this_by(start: true)

  #v(1em)

  #body

  #v(1em)

  #this_by(start: false)
]
