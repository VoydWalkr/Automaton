require('dotenv').config({ path: './.dev.env' });

module.exports = {
  local: {
    mnemonic: process.env.SECRET_MNEMONIC,
  },
};
