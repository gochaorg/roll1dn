roll1dn
===================

Модель
============================

```mermaid
erDiagram
  ROOM {
    u32 room_id
    str name "имя комнаты"
    u8  max_players "максимальное кол-во игроков в комнате"
  }
  PLAYER {
    u32 room_id
    u32 played_id
    str name 
  }
  ROOM }o--o{ PLAYER : room_id
  ROUND {
    u32 round_id
    u32 room_id
    u32 min_players_count "минимальное кол-во игроков для успешного раунда"
    u8 max_winners "максмальное кол-во игроков"
    time_opt started "время начала раунда"
    time_opt finished "время завершения раунда"
    duration timeout "максимальная продолжительность раунда"
  }
  ROOM ||--o{ ROUND : room_id
  ROLL {
    u32 roll_id
    u32 round_id
    u32 player_id
    u8 value
    u32_opt conflict_roll_id    
  }
  PLAYER ||--o{ ROLL : player_id
  ROLL ||--o{ ROUND : round_id
  WINNER {
    u32 round_id
    u32 player
  }
  WINNER }o--|| ROUND : round_id
```

- **ROOM** Игравая комната
- **PLAYER** Игрок
- **ROUND** Раунд
- **ROLL** Бросок кубика
- **WINNER** Победитель

Сценарии
============================

- Регистрация игрока
- Регистрация комнаты
- Старт роунда
- Бросок кубика

Бросок кубика
-----------------------------

```mermaid
stateDiagram-v2
  [*] --> create_round
  create_round --> rolling
  state rolling {
    [*] --> player_roll : incomming http request
    player_roll --> if_round_closed
    if_round_closed --> err_msg : round is closed
    if_round_closed --> add_roll : round is'nt closed
    err_msg --> [*] : output response
    add_roll --> compute_conflict
    compute_conflict --> if_has_conflict
    if_has_conflict --> insert_winner : no conflict
    if_has_conflict --> wait_next : has conflict
    wait_next --> player_roll : output http response
    insert_winner --> close_round
    close_round --> [*] : output http response
  }
```

API
=====================

- users
  - POST `/user/{name}` create user
    - res
      ```json
      {
        "name": "user name"
        "id": 123
      }
      ```
  - GET `/user` list users
    - res
      ```json
      [ { "name": "username a", id:123 } ,
        { "name": "username b", id:124 } ,
        ....
      ]
      ```
  - DELETE `/user/{name}` delete user
  - POST `/user/{name}` rename
    - req
      ```json
      {
        "name": "new name"
        "id": 123
      }
      ```
- room
  - POST `/room/{name}` create room
  - GET `/room` list rooms
  - DELETE `/room/{name}` delete room
  - POST `/room/{name}`
    - req
      ```json
      {
        "name": "new name"
      }
      ```
- room to user  
  - PUT `/welcome/{user_name}/room/{room_name}` associate user with room
  - PUT `/outcome/{user_name}/room/{room_name}` de-associate user with room
