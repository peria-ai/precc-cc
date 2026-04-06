// live-stats.js — Fetch live PRECC stats and inject into pages.
// Loaded by mdbook via additional-js. No rebuild needed when stats update.
(function () {
  var STATS_URL = '/api/stats.json';

  function fmt(val) {
    if (typeof val === 'number' && val >= 1000) return val.toLocaleString();
    return val;
  }

  fetch(STATS_URL)
    .then(function (r) { return r.json(); })
    .then(function (data) {
      // 1. Inject scalar stats into data-stat spans
      document.querySelectorAll('[data-stat]').forEach(function (el) {
        var key = el.getAttribute('data-stat');
        if (key === 'current_version' && data.current_version) {
          el.textContent = 'v' + data.current_version;
          el.style.fontWeight = '600';
        } else if (data[key] !== undefined) {
          el.textContent = fmt(data[key]);
          el.style.fontWeight = '600';
        }
      });

      // 2. Inject measured savings data
      var m = data.measured;
      if (m && m.measurement_count > 0) {
        document.querySelectorAll('[data-measured]').forEach(function (el) {
          var key = el.getAttribute('data-measured');
          if (m[key] !== undefined) {
            el.textContent = fmt(m[key]);
            el.style.fontWeight = '600';
          }
        });
        var measuredDiv = document.getElementById('measured-savings');
        if (measuredDiv) measuredDiv.style.display = '';

        // Per-rewrite-type breakdown
        var rtTable = document.getElementById('rewrite-type-table');
        var rtDiv = document.getElementById('rewrite-type-breakdown');
        if (rtTable && m.by_rewrite_type && m.by_rewrite_type.length > 0) {
          var html = '';
          m.by_rewrite_type.forEach(function (rt) {
            html += '<tr>'
              + '<td><code>' + rt.rewrite_type + '</code></td>'
              + '<td>' + fmt(rt.count) + '</td>'
              + '<td>' + rt.avg_savings_pct + '%</td>'
              + '<td>' + fmt(rt.total_savings_tokens) + '</td>'
              + '</tr>';
          });
          rtTable.querySelector('tbody').innerHTML = html;
          if (rtDiv) rtDiv.style.display = '';
        }
      }

      // 3. Per-version breakdown table
      var versionTable = document.getElementById('version-breakdown');
      if (versionTable && data.by_version && data.by_version.length > 0) {
        var html = '';
        data.by_version.forEach(function (v) {
          html += '<tr>'
            + '<td><strong>v' + v.version + '</strong></td>'
            + '<td>' + (v.users || '?') + '</td>'
            + '<td>' + fmt(v.total_invocations) + '</td>'
            + '<td>' + fmt(v.tokens_saved) + '</td>'
            + '<td><strong>' + v.saving_pct + '%</strong></td>'
            + '</tr>';
        });
        versionTable.querySelector('tbody').innerHTML = html;
        versionTable.style.display = '';
      }
    })
    .catch(function () {
      // Fail silently — fallback text remains
    });
})();
