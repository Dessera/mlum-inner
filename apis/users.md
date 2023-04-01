# 用户数据及操作
## 用户数据项
1. 用户id id string
2. 用户名 username string
3. 密码 password string
4. 性别 gender enum
5. 学历 education enum
6. 学校 school list(string)
7. 专业 major string
8. 个人简介 description string
9. 我的关注 following list(string)
10. 我参与过的讨论 participated list(string)
11. 我发表的讨论 published list(string)
12. 我的收藏 collection list(string)
13. token token string
14. 用户头像 avatar string(url)
15. 手机号码 phone string
16. 邮箱 email string
17. 注册时间 register_time timestamp

## 用户操作
1. 注册 register
2. 登录 login
3. 登出 logout
4. 获取用户信息 getUserInfo
5. 修改用户信息 modifyUserInfo

## 用户操作描述 /users
### 注册 /register
#### 请求 POST
1. 用户名 username string
2. 密码 password string
3. 手机号码 phone option(string)
4. 邮箱 email option(string)
#### 返回
1. token string

### 登录 /login
#### 请求 POST
1. 用户名 username string
2. 密码 password string
3. 手机号码 phone option(string)
4. 邮箱 email option(string)
#### 返回
1. token string

### 登出 /logout
#### 请求 POST
1. token string
#### 返回
1. 无

### 获取用户信息 /profile
#### 请求 GET
1. 用户名 username string
#### 返回
1. 用户数据项
#### 注意
返回的数据中，password字段和token字段为空

### 修改用户信息 /update
#### 请求 PUT
1. 用户数据项
#### 返回
1. 用户数据项
#### 注意
发送的数据中，需要token字段，
修改的数据中，password字段和token字段为空

### 删除用户 /delete
#### 请求 DELETE
1. 用户名 username string
#### 返回
1. 无
#### 注意
并不会真正删除用户，只是将用户的数据项中的is_deprecated字段设置为true

### 验证用户 /verify
#### 请求 POST
1. 用户名 username string
2. token string
#### 返回
1. 无