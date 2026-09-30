import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { errorMessages } from '../api/client';
import { useSession } from '../session/SessionContext';
import { ErrorMessages } from './ErrorMessages';

export function LoginForm() {
  const session = useSession();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errors, setErrors] = useState<string[]>([]);

  const submit = async (e: React.MouseEvent) => {
    e.preventDefault();
    setErrors([]);
    try {
      await session.logIn(email, password);
      navigate('/');
    } catch (err) {
      setErrors(errorMessages(err));
    }
  };

  return (
    <>
      <h1 className="text-xs-center">Sign in</h1>
      <p className="text-xs-center">
        <Link to="/register" data-test-register-link>
          Need an account?
        </Link>
      </p>
      <ErrorMessages errors={errors} />
      <form onSubmit={(e) => e.preventDefault()}>
        <fieldset className="form-group">
          <input
            className="form-control form-control-lg"
            type="text"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            data-test-login-email
          />
        </fieldset>
        <fieldset className="form-group">
          <input
            className="form-control form-control-lg"
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            data-test-login-password
          />
        </fieldset>
        <button
          className="btn btn-lg btn-primary pull-xs-right"
          onClick={submit}
          disabled={!email || !password}
          type="button"
          data-test-login-button
        >
          Sign in
        </button>
      </form>
    </>
  );
}
