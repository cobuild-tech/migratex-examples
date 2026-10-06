import { useReducer } from 'react';
import { BillInput } from './components/BillInput';
import { PeopleInput } from './components/PeopleInput';
import { ResultsPanel } from './components/ResultsPanel';
import { TipSelector } from './components/TipSelector';
import logo from './images/logo.svg';
import { calculateSplit } from './lib/split';
import { initialState, isDirty, peopleIsZero, reducer, tipPercent } from './state';
import './styles/styles.css';
import './styles/root.css';

export function App() {
  const [state, dispatch] = useReducer(reducer, initialState);
  const split =
    state.bill === '' || state.people === ''
      ? null
      : calculateSplit(parseFloat(state.bill), tipPercent(state), Number(state.people));

  return (
    <>
      <div className="logo">
        <div className="logo-sub"><img src={logo} alt="Splitter" /></div>
      </div>

      <section className="main-body">
        <div className="main">
          <div className="inputs">
            <BillInput value={state.bill} onChange={(value) => dispatch({ type: 'setBill', value })} />
            <TipSelector
              tip={state.tip}
              customTip={state.customTip}
              onSelectPreset={(pct) => dispatch({ type: 'selectPreset', pct })}
              onCustomTipChange={(value) => dispatch({ type: 'setCustomTip', value })}
            />
            <PeopleInput
              value={state.people}
              invalid={peopleIsZero(state)}
              onChange={(value) => dispatch({ type: 'setPeople', value })}
            />
          </div>

          <ResultsPanel split={split} canReset={isDirty(state)} onReset={() => dispatch({ type: 'reset' })} />
        </div>
      </section>
    </>
  );
}
