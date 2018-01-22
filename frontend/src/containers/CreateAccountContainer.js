

const mapDispatchToProps = dispatch => ({
  createAccount: (userInfo) => {
    dispatch(doCreateAccount(userInfo));
  },
});
