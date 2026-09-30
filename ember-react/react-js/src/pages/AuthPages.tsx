import type { ReactNode } from 'react';
import { LoginForm } from '../components/LoginForm';
import { RegisterForm } from '../components/RegisterForm';
import { SettingsForm } from '../components/SettingsForm';
import { useSession } from '../session/SessionContext';

function Centered({ page, children }: { page: string; children: ReactNode }) {
  return (
    <div className={page}>
      <div className="container page">
        <div className="row">
          <div className="col-md-6 offset-md-3 col-xs-12">{children}</div>
        </div>
      </div>
    </div>
  );
}

export const LoginPage = () => (
  <Centered page="auth-page">
    <LoginForm />
  </Centered>
);

export const RegisterPage = () => (
  <Centered page="auth-page">
    <RegisterForm />
  </Centered>
);

export function SettingsPage() {
  const { user } = useSession();
  return <Centered page="settings-page">{user && <SettingsForm user={user} />}</Centered>;
}
