# curl -i --request POST \
#   --url http://127.0.0.1:8000/api/auth \
#   --header 'content-type: application/json' \
#   --data '{"login": "login","password":"password"}'

curl -i --request POST \
  --url http://127.0.0.1:8000/register/request \
  -d "email=email2@toto.fr&username=toto&password=totop"
#   --header 'content-type: application/json' \
#  --data '{"email": "email2@toto.fr", "username":"toto", "password":"totop"}'
