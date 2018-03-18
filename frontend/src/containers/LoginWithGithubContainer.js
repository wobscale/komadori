import { connect } from 'react-redux';
import { doHandleAuth } from '../actions';
import GithubLogin from '../components/GithubLogin';

const mapDispatchToProps = dispatch => ({
  onAuth: (githubInfo) => {
    dispatch(doHandleAuth('github', githubInfo));
  },
});

const LoginWithGithub = connect(
  null,
  mapDispatchToProps,
)(GithubLogin);

export default LoginWithGithub;
