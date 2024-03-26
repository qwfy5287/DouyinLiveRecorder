



登录接口
curl --location --request POST 'http://localhost:20248/douyin-user/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "phone": "string",
    "password": "string"
}'

登录成功返回信息

{
    "success": true,
    "code": 20000,
    "message": "登录成功",
    "count": 0,
    "records": []
}

登录失败返回信息 01

{
    "success": true,
    "code": 410,
    "message": "用户名或密码错误",
    "count": 0,
    "records": []
}