import { useState } from 'react';
import { Link } from 'react-router-dom';
import { errorMessages } from '../api/client';
import type { Article, Comment as CommentType } from '../api/types';
import { useAddComment, useComments, useDeleteComment } from '../hooks/queries';
import { formatDate } from '../lib/formatDate';
import { useSession } from '../session/SessionContext';
import { ErrorMessages } from './ErrorMessages';
import { profilePath } from '../lib/paths';

function CommentForm({ onAdd, isPosting }: { onAdd(body: string): Promise<unknown>; isPosting: boolean }) {
  const { user } = useSession();
  const [body, setBody] = useState('');

  const submit = async (e: React.MouseEvent) => {
    e.preventDefault();
    try {
      await onAdd(body);
      setBody(''); // only clear once posted, so a failure keeps the draft
    } catch {
      // reported by CommentsSection
    }
  };

  return (
    <form className="card comment-form" onSubmit={(e) => e.preventDefault()}>
      <div className="card-block">
        <textarea
          className="form-control"
          placeholder="Write a comment..."
          rows={3}
          value={body}
          onChange={(e) => setBody(e.target.value)}
          data-test-article-comment-textarea
        />
      </div>
      <div className="card-footer">
        <img src={user?.image ?? undefined} className="comment-author-img" alt={user?.username} />
        <button
          className="btn btn-sm btn-primary"
          disabled={!body || isPosting}
          onClick={submit}
          type="button"
          data-test-article-comment-button
        >
          Post Comment
        </button>
      </div>
    </form>
  );
}

function Comment({ comment, onDelete }: { comment: CommentType; onDelete(): void }) {
  const { user } = useSession();
  const profileUrl = profilePath(comment.author.username);

  return (
    <div className="card" data-test-article-comment>
      <div className="card-block">
        <p className="card-text" data-test-article-comment-body>
          {comment.body}
        </p>
      </div>
      <div className="card-footer">
        <Link to={profileUrl} className="comment-author">
          <img src={comment.author.image ?? undefined} className="comment-author-img" alt={comment.author.username} />
        </Link>
        &nbsp;
        <Link to={profileUrl} className="comment-author">
          {comment.author.username}
        </Link>
        <span className="date-posted">{formatDate(comment.updatedAt)}</span>
        {user?.username === comment.author.username && (
          <a
            href=""
            className="mod-options"
            onClick={(e) => {
              e.preventDefault();
              onDelete();
            }}
            data-test-article-comment-delete-button
          >
            <i className="ion-trash-a"></i>
          </a>
        )}
      </div>
    </div>
  );
}

export function CommentsSection({ article }: { article: Article }) {
  const { isLoggedIn } = useSession();
  const { data: comments = [], isLoading } = useComments(article.slug);
  const addComment = useAddComment(article.slug);
  const deleteComment = useDeleteComment(article.slug);
  const [errors, setErrors] = useState<string[]>([]);

  const report = (e: unknown) => {
    setErrors(errorMessages(e));
    throw e;
  };
  const add = (body: string) => {
    setErrors([]);
    return addComment.mutateAsync(body).catch(report);
  };
  const remove = (id: CommentType['id']) => {
    setErrors([]);
    deleteComment.mutateAsync(id).catch(report).catch(() => {});
  };

  return (
    <>
      <ErrorMessages errors={errors} itemAttr="data-test-comment-error" />
      {isLoggedIn && <CommentForm onAdd={add} isPosting={addComment.isPending} />}
      {isLoading ? (
        <p>Loading comments...</p>
      ) : (
        comments.map((comment) => (
          <Comment key={comment.id} comment={comment} onDelete={() => remove(comment.id)} />
        ))
      )}
    </>
  );
}
