#let format-date(yyyy-mm-dd) = {
  let timespans = yyyy-mm-dd.split("-").map(timespan => {int(timespan)})
  let date = datetime(
    year: timespans.at(0), 
    month: timespans.at(1), 
    day: timespans.at(2)
  )
  date.display("[month repr:long] [day padding:none], [year]")
}

// #format-date("2003-10-02")