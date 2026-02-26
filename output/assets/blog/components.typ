#let by(start: bool, name: str, email: str, date: datetime) = [
  #align(if start { left } else { right })[
    #name | #email

    #date.display()
  ]
]

#let post(title: str, name: str, email: str, date: datetime, body) = [
  // #set page(height: 10 * 297mm)

  #let this_by(start: bool) = by(
    start: start,
    name: name,
    email: email,
    date: date,
  )

  = #title

  #v(0.5em)

  #this_by(start: true)

  #v(1em)

  #body

  \

  #v(1em)

  \

  #this_by(start: false)
]
