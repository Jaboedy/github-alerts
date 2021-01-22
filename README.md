# github-alerts
Streamlabs alerts triggered by github webhook events

# About this Project
This is my first project built with rust and as such is incredibly lacking.  
It will itermittently be updated as I learn more about the language.

# How to use  
There are almost assuredly better options than this repo currently, however feel free to use it if you like


I am currently hosting this app on Heroku, so main.rs is currently set up to use a provided PORT env variable.  
This app uses `dotenv` to handle environment variables.  In order to run properly as is the `.env` file must contain:  
* `HOST`: Mine is set to `0.0.0.0` for use with Heroku
* `STREAMLABS_OAUTH`: Oauth token requiring the scope `alerts.create`
* `PORT`: If you are also hosting on Heroku this will be provided and won't need to be set in the `.env` file
