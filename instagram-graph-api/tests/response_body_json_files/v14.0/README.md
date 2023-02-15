## Ok json files

| File                                                             | Url                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ig_media_17946328927974136__comments__reading.json               | GET /v15.0/17946328927974136/comments?fields=from,hidden,id,like_count,text,timestamp,username,replies{from,hidden,id,like_count,parent_id,text,timestamp,username}&limit=30                                                                                                                                                                                                  |
| ig_media_17946328927974136__comments__creating.json              | POST /v15.0/17946328927974136/comments?message=test1&fields=from,hidden,id,like_count,text,timestamp,username                                                                                                                                                                                                                                                                 |
| ig_comment_17897793971532270__replies__creating.json             | POST /v15.0/17897793971532270/replies?message=test2&fields=from,hidden,id,like_count,parent_id,text,timestamp,username                                                                                                                                                                                                                                                        |
| ig_comment_0__updating__hide_sample.json                         | POST /v14.0/0?hide=true                                                                                                                                                                                                                                                                                                                                                       |
| ig_comment_0__deleting__sample.json                              | DELETE /v14.0/0                                                                                                                                                                                                                                                                                                                                                               |
| ig_media_0__updating__comment_enabled_sample.json                | POST /v14.0/0?comment_enabled=true                                                                                                                                                                                                                                                                                                                                            |
| ig_user_0__business_discovery__reading__username_bluebottle.json | GET /v15.0/17841406427775093?fields=business_discovery.username(bluebottle){biography,id,ig_id,followers_count,follows_count,media_count,name,profile_picture_url,username,website,media.since(1640966400).limit(30){caption,comments_count,id,like_count,media_product_type,media_type,media_url,permalink,timestamp,children{id,media_type,media_url,permalink,timestamp}}} |
| ig_user_0__media__creating__image.json                           | POST /v15.0/17841406427775093/media?fields=id,status,status_code&image_url=xxxxxx&is_carousel_item=false&caption=Test&location_id=106487912721749&user_tags=%5B%7B%22x%22%3A0.5%2C%22y%22%3A0.5%2C%22username%22%3A%22heyongpeng%22%7D%5D                                                                                                                                     |
| ig_user_0__media__creating__video.json                           | ditto                                                                                                                                                                                                                                                                                                                                                                         |
| ig_user_0__media__creating__carousel.json                        | ditto                                                                                                                                                                                                                                                                                                                                                                         |

## Err json files

| File                                                             | StatusCode | Url                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| err__ig_user_0__media__creating__usertag_username_private.json   | 400        | POST /v15.0/17841406427775093/media?fields=id,status,status_code&image_url=xxxxxx&is_carousel_item=false&caption=Test&location_id=106487912721749&user_tags=%5B%7B%22x%22%3A0.5%2C%22y%22%3A0.5%2C%22username%22%3A%22heyongpeng%22%7D%5D |
| err__ig_user_0__media__creating__image_aspect_ratio_invalid.json | 400        | ditto                                                                                                                                                                                                                                     |
| err__ig_user_0__media__creating__image_url_invalid.json          | 400        | ditto                                                                                                                                                                                                                                     |