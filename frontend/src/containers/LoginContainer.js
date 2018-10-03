import PropTypes from 'prop-types';
import Config from '../config';
import LoginComponent from '../components/LoginPage';

const LoginPage = props => LoginComponent({
  location: props.location,
  providers: Config.loginProviders.map((p) => {
    switch (p) {
      case 'github':
        return {
          name: 'Github',
          pathname: '/github/login',
        };
      case 'local':
        return {
          name: 'Dev Auth',
          pathname: '/dev/login',
        };
      default:
        alert(`invalid provider: ${p}`);
        return {};
    }
  }),
});
LoginPage.propTypes = {
  location: PropTypes.object.isRequired,
};

export default LoginPage;
