require('dotenv/config');

const handleCommand = require('../src/index');
const express = require('express');

const app = express();

app.use(express.json());

app.all('*', handleCommand);

app.listen(process.env.PORT || 3000, () => console.log('Listening'));
