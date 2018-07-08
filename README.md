# <img src="images/logo_big.png" height="150" alt="sudoku" />
A modern, API-based Sudoku app.

## Introduction

Our goal is to make the best Sudoku app on the web.

There are a lot of Sudoku games on the web, but none of them well built nor with good design.
We want to make our take modern, secure, well-designed and open to everyone – it works well on all available platforms and screens.

`Sudoku` consists of a public API, web app, and in the future – a mobile app.

# <img src="images/progress.png" alt="Progress" height="280"/>

## API

<sub>Subject to change and move elsewhere with rest of API documentation.</sub>

  - USERS
    - Register a new user
      > POST `sudoku/api/register`
    - Validate users login and password
      > POST `sudoku/api/login`
    - Return the number of boards solved by a user (ordered by duration and difficulty)
      > GET `sudoku/api/getScore?user=username`
    - Compare users scores in a leaderboard
      > GET `sudoku/api/getLeaderboard`
  - GAME
    - Generate new sudoku boards
      > GET `sudoku/api/generateBoard?difficulty=1&variant=0`
    - Validate completed ones
      > POST `sudoku/api/validateBoard`

After a game is won, the player is awarded points, calculated as follows:

```c
points = difficulty * (3000 / solvingDuration + 30)
```

Where `difficulty` ϵ {1, 2, 3} and `solvingDuration` is in seconds.

All endpoint data is returned in  **JSON**.

## Database

<sub>Here temporarily, subject to change and move elsewhere with rest of internal design documents.</sub>

### USERS
|  id |    name    |      password     |      seed      |          email         |  role | points |
|-----|------------|-------------------|----------------|------------------------|-------|-------:|
| `0` |  karolsw3  | 14c80afe290ba6dE1 | 4FaCc948fab1B2 | karol.sw3@gmail.com    | admin |    983 |
| `1` |     bob    | 44f80cfeC53Aa4d71 | 911Cd9D82abeC5 | bob@blob.com           | user  |   3984 |
| `2` | sudokuPapa | 5ff34cac003ca4c90 | 3FFaDa3fe8be47 | noobfrom@minecraft.net | user  |  45682 |
etc.

### GAMES

|  id | userId | difficulty | duration |    date    |
|-----|-------:|-----------:|---------:|------------|
| `0` |    0   |      1     |    325   | 21-04-2018 |
| `1` |    0   |      1     |    315   | 21-04-2018 |
| `2` |    2   |      1     |     62   | 12-06-2018 |
| `3` |    1   |      2     |    142   | 03-12-2017 |
etc.

## Contributing

We are open to contributors, so don't hesitate to make a pull request!

## Licence

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details
