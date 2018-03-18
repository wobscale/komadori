import { connect } from 'react-redux';
import { doCreateAccount } from '../actions';
import CreateAccountReact from '../components/CreateAccount';

const mapStateToProps = (state) => {
  const { user, partialUser } = state;
  if (user.loggedIn) {
    // Shouldn't happen
    // TODO: dispatch error
    alert('no');
    return {};
  }
  return {
    partialUser: partialUser.partialUser,
  };
};

const mapDispatchToProps = dispatch => ({
  createAccount: (userInfo) => {
    dispatch(doCreateAccount(userInfo));
  },
});

const CreateAccount = connect(
  mapStateToProps,
  mapDispatchToProps,
)(CreateAccountReact);

export default CreateAccount;
