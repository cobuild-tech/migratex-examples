import { useState } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { errorMessages } from '../api/client';
import * as api from '../api/endpoints';
import type { User } from '../api/types';
import { useSession } from '../session/SessionContext';
import { ErrorMessages } from './ErrorMessages';

type Fields = { image: string; username: string; bio: string; email: string; password: string };

const fieldsFrom = (user: User): Fields => ({
  image: user.image ?? '',
  username: user.username,
  bio: user.bio ?? '',
  email: user.email,
  password: '',
});

export function SettingsForm({ user }: { user: User }) {
  const session = useSession();
  const queryClient = useQueryClient();
  const [initial, setInitial] = useState(() => fieldsFrom(user));
  const [form, setForm] = useState(initial);
  const [errors, setErrors] = useState<string[]>([]);
  const [isSaving, setIsSaving] = useState(false);

  const isDirty = (Object.keys(form) as (keyof Fields)[]).some((key) => form[key] !== initial[key]);

  const update = (field: keyof Fields) => (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
    setForm((f) => ({ ...f, [field]: e.target.value }));

  const submit = async (e: React.MouseEvent) => {
    e.preventDefault();
    setErrors([]);
    setIsSaving(true);
    const { password, ...rest } = form;
    try {
      const updated = await api.updateUser(password ? form : rest);
      session.setUser(updated);
      setInitial(form);
      // Our name/bio/image is embedded in cached profiles, article authors and comments.
      if (updated.username !== initial.username) {
        queryClient.removeQueries({ queryKey: ['profile', initial.username] });
      }
      queryClient.invalidateQueries();
    } catch (err) {
      setErrors(errorMessages(err));
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <>
      <h1 className="text-xs-center">Your Settings</h1>
      <ErrorMessages errors={errors} itemAttr="data-test-settings-form-error-item" />
      <form onSubmit={(e) => e.preventDefault()}>
        <fieldset>
          <fieldset className="form-group">
            <input
              className="form-control"
              type="text"
              placeholder="URL of profile picture"
              value={form.image}
              onChange={update('image')}
              data-test-settings-form-input-image
            />
          </fieldset>
          <fieldset className="form-group">
            <input
              className="form-control form-control-lg"
              type="text"
              placeholder="Your Name"
              value={form.username}
              onChange={update('username')}
              data-test-settings-form-input-username
            />
          </fieldset>
          <fieldset className="form-group">
            <textarea
              className="form-control form-control-lg"
              rows={8}
              placeholder="Short bio about you"
              value={form.bio}
              onChange={update('bio')}
              data-test-settings-form-input-bio
            />
          </fieldset>
          <fieldset className="form-group">
            <input
              className="form-control form-control-lg"
              type="text"
              placeholder="Email"
              value={form.email}
              onChange={update('email')}
              data-test-settings-form-input-email
            />
          </fieldset>
          <fieldset className="form-group">
            <input
              className="form-control form-control-lg"
              type="password"
              placeholder="Password"
              value={form.password}
              onChange={update('password')}
              data-test-settings-form-input-password
            />
          </fieldset>
          <button
            className="btn btn-lg btn-primary pull-xs-right"
            disabled={!isDirty || isSaving}
            onClick={submit}
            type="button"
            data-test-settings-form-button
          >
            Update Settings
          </button>
        </fieldset>
      </form>
    </>
  );
}
