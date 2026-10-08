// Small DOM adapter: keyed patches, local input state, actions, and reconnect.
(() => {
  const find = key => document.querySelector(`[data-mkey="${CSS.escape(key)}"]`);
  const tokenMeta = document.querySelector('meta[name="monolith-token"]');
  let revision = -1;
  let busy = false;
  const status = text => { find('connection').textContent = text; };
  const error = text => { find('error').textContent = text; };
  // Reconnect snapshots reuse nodes. Draft values, focus, and selection stay local.
  function reconcile(current, incoming) {
    const localInput = current.matches('input[type="text"]');
    for (const attr of [...current.attributes]) {
      if (!incoming.hasAttribute(attr.name) && !(localInput && attr.name === 'value')) current.removeAttribute(attr.name);
    }
    for (const attr of incoming.attributes) {
      if (!(localInput && attr.name === 'value')) current.setAttribute(attr.name, attr.value);
    }
    if (current.matches('input[type="checkbox"]')) current.checked = incoming.checked;
    if (!incoming.children.length) {
      if (!current.matches('input')) current.textContent = incoming.textContent;
      return current;
    }
    const wanted = new Set();
    let previous = null;
    for (const child of [...incoming.children]) {
      const key = child.dataset.mkey;
      let target = [...current.children].find(el => !wanted.has(el) && (key
        ? el.dataset.mkey === key
        : !el.dataset.mkey && el.tagName === child.tagName && el.getAttribute('name') === child.getAttribute('name')));
      if (target && target.tagName === child.tagName) reconcile(target, child);
      else target = child;
      wanted.add(target);
      const before = previous ? previous.nextElementSibling : current.firstElementChild;
      if (target !== before) current.insertBefore(target, before);
      previous = target;
    }
    for (const child of [...current.children]) if (!wanted.has(child)) child.remove();
    return current;
  }
  function position(node, op) {
    const parent = find(op.parent);
    if (!parent) throw new Error('Missing patch parent');
    const after = op.after ? find(op.after) : null;
    parent.insertBefore(node, after ? after.nextElementSibling : parent.firstElementChild);
  }
  function apply(op) {
    let node = find(op.key);
    if (op.op === 'insert') {
      const template = document.createElement('template');
      template.innerHTML = op.html;
      const incoming = template.content.firstElementChild;
      if (node) reconcile(node, incoming);
      else position(incoming, op);
      return;
    }
    if (!node) throw new Error('Missing patch node');
    if (op.op === 'remove') node.remove();
    else if (op.op === 'move') position(node, op);
    else if (op.op === 'text') node.textContent = op.value;
    else if (op.op === 'checked') node.checked = op.value;
    else if (op.op === 'hidden') node.hidden = op.value;
    else if (op.op === 'class') node.className = op.value;
    else if (op.op === 'label') node.setAttribute('aria-label', op.value);
    else if (op.op === 'placeholder') node.placeholder = op.value;
    else throw new Error('Unknown UI operation');
  }
  const events = new EventSource('/events');
  events.onmessage = event => {
    try {
      const message = JSON.parse(event.data);
      if (!message.reset && message.revision <= revision) return;
      if (!message.reset && message.revision !== revision + 1) throw new Error('Missed UI revision');
      message.ops.forEach(apply);
      revision = message.revision;
      tokenMeta.content = message.token;
      document.querySelectorAll('input[name="token"]').forEach(el => { el.value = message.token; });
      status('Live');
    } catch (_) { events.close(); status('Reload to reconnect'); error('The view could not synchronize. Reload this page.'); }
  };
  events.onerror = () => status('Reconnecting…');
  async function action(name, id, title) {
    if (busy) return false;
    busy = true;
    error('');
    try {
      const body = new URLSearchParams({ token: tokenMeta.content, action: name });
      if (id) body.set('id', id);
      if (title !== undefined) body.set('title', title);
      const response = await fetch('/action', { method: 'POST', headers: { Accept: 'application/json', 'Content-Type': 'application/x-www-form-urlencoded' }, body });
      if (!response.ok) throw new Error(response.status === 422 ? 'Enter a task of 1–240 bytes.' : 'Could not save this change.');
      return true;
    } catch (e) { error(e.message); return false; }
    finally { busy = false; }
  }
  document.addEventListener('submit', async event => {
    if (event.target.dataset.mkey !== 'add-form') return;
    event.preventDefault();
    const draft = find('draft');
    const submitted = draft.value;
    if (await action('add', null, submitted)) { if (draft.value === submitted) draft.value = ''; draft.focus(); }
  });
  document.addEventListener('change', async event => {
    const target = event.target;
    if (target.dataset.action !== 'toggle') return;
    const previous = !target.checked;
    if (!(await action('toggle', target.dataset.id))) target.checked = previous;
  });
  document.addEventListener('click', event => {
    const target = event.target.closest('[data-action="delete"]');
    if (target) action('delete', target.dataset.id);
  });
})();
