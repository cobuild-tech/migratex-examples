import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { ApiError } from '../api/client';
import { ArticleMeta } from '../components/ArticleMeta';
import { CommentsSection } from '../components/CommentsSection';
import { ErrorMessages } from '../components/ErrorMessages';
import { useArticle, useDeleteArticle } from '../hooks/queries';
import { renderMarkdown } from '../lib/markdown';
import { NotFoundPage } from './NotFoundPage';

export function ArticlePage() {
  const { id = '' } = useParams();
  const navigate = useNavigate();
  const { data: article, isError } = useArticle(id);
  // One delete mutation shared by both meta rows, so a failure is reported once.
  const deleteArticle = useDeleteArticle();
  const [deleteErrors, setDeleteErrors] = useState<string[]>([]);

  if (isError) return <NotFoundPage />;
  if (!article) return null;

  const onDelete = async () => {
    setDeleteErrors([]);
    try {
      await deleteArticle.mutateAsync(article.slug);
      navigate('/');
    } catch (e) {
      setDeleteErrors(
        e instanceof ApiError && e.errors.length ? e.errors : ['Could not delete the article. Please try again.'],
      );
    }
  };
  const meta = <ArticleMeta article={article} onDelete={onDelete} isDeleting={deleteArticle.isPending} />;

  return (
    <div className="article-page">
      <div className="banner">
        <div className="container">
          <h1 data-test-article-title>{article.title}</h1>
          {meta}
          <ErrorMessages errors={deleteErrors} itemAttr="data-test-delete-article-error" />
        </div>
      </div>
      <div className="container page">
        <div className="row article-content">
          <div
            className="col-md-12"
            data-test-article-body
            dangerouslySetInnerHTML={{ __html: renderMarkdown(article.body) }}
          />
        </div>
        <hr />
        <div className="article-actions">
          {meta}
        </div>
        <div className="row">
          <div className="col-xs-12 col-md-8 offset-md-2">
            <CommentsSection article={article} />
          </div>
        </div>
      </div>
    </div>
  );
}
