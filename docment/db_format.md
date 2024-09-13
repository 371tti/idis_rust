## Meta data format

変更予定

```json
{
	"_id": Object,
	"name": String,
	"path": String,
	"id": RUID/*metadata*/,
	"links": [(RUID/*user*/, RUID/*metadata*/), ...],
	"about": String,
	"w_perm": [RUID/*user,permGroup*/, ...],
	"r_perm": [RUID/*user,permGroup*/, ...],
	"e_perm": [RUID/*user,permGroup*/, ...],
	"log": json,
	"event": json,
	"viws": int,
	"reaction": {RUID/*object*/:[RUID/*user*/, ...], ...},
	"reaction-count": {RUID/*object*/: int, ...}
}
```

* RUID is original id format...

1. `_id`: mongo_db_docment_id-cna't edit
2. `name`: file or folder name
3. `path`: file or folder path-can't edit
4. `id`: user RUID. include data_type, create_time and data_id-can't edit
5. `links`: under file and folder path list if deta_type is folder
6. `about`: File Description and Summary
7. `w_perm`: List of write permissions
8. `r_perm`: List of read permission
9. `e_perm`: List of edit permission
10. `log`: edit log of data. key is RUID  and val is log of string
11. `event`: Executes a command when an action occurs
12. `viw`: Impressions-can't edit
13. `reaction`: reactions-can't edit
14. `reaction-count`: reaction count-can't edit

## User System data format

```json
{
   'account-type': int,
   'id': String,
   'name': String,
   'password': String,
   'RUID': RUID/*user*/,
}
```
