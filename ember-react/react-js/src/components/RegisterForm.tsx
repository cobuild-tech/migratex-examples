import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { errorMessages } from '../api/client';
import { useSession } from '../session/SessionContext';
import { ErrorMessages } from './ErrorMessages';

export function RegisterForm() {
  const session = useSession();
  const navigate = useNavigate();
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errors, setErrors] = useState<string[]>([]);

  const submit = async (e: React.MouseEvent) => {
    e.preventDefault();
    setErrors([]);
    try {
      await session.register(username, email, password);
      navigate('/');
    } catch (err) {
      setErrors(errorMessages(err));
    }
  };

  return (
    <>
      <h1 className="text-xs-center">Sign up</h1>
      <p className="text-xs-center">
        <Link to="/login" data-test-login-link>
          Have an account?
        </Link>
      </p>
      <ErrorMessages errors={errors} />
      <form onSubmit={(e) => e.preventDefault()}>
        <fieldset className="form-group">
          <input
            className="form-control form-control-lg"
            type="text"
            placeholder="Your Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            data-test-register-username
          />
        </fieldset>
        <fieldset className="form-group">
          <input
            className="form-control form-control-lg"
            type="text"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            data-test-register-email
          />
        </fieldset>
        <fieldset className="form-group">
          <input
            className="form-control form-control-lg"
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            data-test-register-password
          />
        </fieldset>
        <button
          className="btn btn-lg btn-primary pull-xs-right"
          onClick={submit}
          disabled={!username || !email || !password}
          type="button"
          data-test-register-button
        >
          Sign up
        </button>
      </form>
    </>
  );
}
