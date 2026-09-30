import { useEffect, useRef, useState } from 'react';
import { useBlocker, useNavigate } from 'react-router-dom';
import { errorMessages } from '../api/client';
import type { Article, ArticleInput } from '../api/types';
import { useSaveArticle } from '../hooks/queries';
import { ErrorMessages } from './ErrorMessages';
import { articlePath } from '../lib/paths';

const parseTags = (raw: string) => raw.split(/\s+/).filter(Boolean);

export function ArticleForm({ article }: { article?: Article }) {
  const navigate = useNavigate();
  const save = useSaveArticle(article?.slug);
  const initial = useRef({
    title: article?.title ?? '',
    description: article?.description ?? '',
    body: article?.body ?? '',
    rawTagList: article?.tagList.join(' ') ?? '',
  });
  const [form, setForm] = useState(initial.current);
  const [errors, setErrors] = useState<string[]>([]);
  const saved = useRef(false);

  const isDirty =
    form.title !== initial.current.title ||
    form.description !== initial.current.description ||
    form.body !== initial.current.body ||
    parseTags(form.rawTagList).join(' ') !== parseTags(initial.current.rawTagList).join(' ');

  // Editing an existing article prompts before leaving with unsaved changes.
  const blocker = useBlocker(
    ({ currentLocation, nextLocation }) =>
      !!article && isDirty && !saved.current && !save.isPending && currentLocation.pathname !== nextLocation.pathname,
  );
  useEffect(() => {
    if (blocker.state !== 'blocked') return;
    if (window.confirm("You haven't saved your changes. Are you sure you want to leave the page?")) {
      blocker.proceed();
    } else {
      blocker.reset();
    }
  }, [blocker]);

  const update = (field: keyof typeof form) => (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
    setForm((f) => ({ ...f, [field]: e.target.value }));

  const publish = async () => {
    setErrors([]);
    const input: ArticleInput = {
      title: form.title,
      description: form.description,
      body: form.body,
      tagList: parseTags(form.rawTagList),
    };
    try {
      const result = await save.mutateAsync(input);
      saved.current = true;
      navigate(articlePath(result.slug));
    } catch (e) {
      setErrors(errorMessages(e));
    }
  };

  return (
    <>
      <ErrorMessages errors={errors} itemAttr="data-test-article-form-error-item" />
      <form onSubmit={(e) => e.preventDefault()}>
        <fieldset>
          <fieldset className="form-group">
            <input
              type="text"
              className="form-control form-control-lg"
              placeholder="Article Title"
              value={form.title}
              onChange={update('title')}
              data-test-article-form-input-title
            />
          </fieldset>
          <fieldset className="form-group">
            <input
              type="text"
              className="form-control"
              placeholder="What's this article about?"
              value={form.description}
              onChange={update('description')}
              data-test-article-form-input-description
            />
          </fieldset>
          <fieldset className="form-group">
            <textarea
              className="form-control"
              rows={8}
              placeholder="Write your article (in markdown)"
              value={form.body}
              onChange={update('body')}
              data-test-article-form-input-body
            />
          </fieldset>
          <fieldset className="form-group">
            <input
              type="text"
              className="form-control"
              placeholder="Enter tags"
              value={form.rawTagList}
              onChange={update('rawTagList')}
              data-test-article-form-input-tags
            />
            <div className="tag-list"></div>
          </fieldset>
          <button
            className="btn btn-lg pull-xs-right btn-primary"
            type="button"
            disabled={!isDirty || save.isPending}
            onClick={publish}
            data-test-article-form-submit-button
          >
            {save.isPending ? 'Saving' : 'Publish'} Article
          </button>
        </fieldset>
      </form>
    </>
  );
}
