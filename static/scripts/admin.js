$('#signout').click(() => {
  $.post('/admin_signout').done(() => location.href = '/');
});

// Event handlers to close dialogs
$('dialog').each(function () {
  $(`#${this.id} .close, #${this.id} + .overlay`).click(() => {
    $(this).hide('slow', () => $(`#${this.id} + .overlay`).hide());
  });
});

$('.reject, .accept').click(function () {
  let is_report = $(this).parent().parent().parent().attr('id') === 'reports';
  let dialog = $(this).hasClass('reject') ? 'confirmation' : is_report ? 'override' : 'edit_override';
  let id = $(this).parent().parent().attr('class');
  $(`#${dialog} + .overlay`).show();
  $(`#${dialog}`).attr('data-id', id).attr('data-type', is_report ? 'report' : 'override').show('slow');
  $(`#${dialog} .template`).text(is_report ? 'report' : 'override');
  if (is_report && $(this).hasClass('accept')) {
    $('#question').val($(this).parent().siblings('h2').text());
    for (let property of ['translation', 'reading']) {
      $(`#${property}`).val($(`#reports .${id} .${property}:first-of-type`).text());
    }
    for (let property of ['report_type', 'comment']) {
      let value = $(`#reports .${id} .${property}`).text();
      // Hide the comment div if there is no comment
      // This doesn't apply for report_type
      if (value.length) {
        $(`#${dialog} .${property}`).show();
        $(`#${property}`).text(value);
      } else {
        $(`#${dialog} .${property}`).hide();
      }
    }
    let suggested = $(`#reports .${id} .suggested`).text();
    if (suggested.length) {
      $(`#${dialog} .suggested`).show();
      $('#suggested').text(suggested);
    } else {
      $(`#${dialog} .suggested`).hide();
    }
  } else if ($(this).hasClass('accept')) {
    $('#edit_override .question').text($(this).parent().siblings('h2').text());
    $('#edit_override .translation').text($(`#overrides .${id} .translation`).text());
    $('#edit_override .reading').text($(`#overrides .${id} .reading`).text());
    $('#edit_override .override_type').text($(`#overrides .${id} .override_type`).text());
    $('#value').val($(`#overrides .${id} .value`).text());
    $('#primary').prop('checked', !!$(`#overrides .${id} .primary`).length);
  }
});

$('#confirmation button:last-child').click(() => {
  $('#confirmation button').prop('disabled', true);
  $.post('/delete_' + $('#confirmation').attr('data-type'), {
    value: $('#confirmation').attr('data-id'),
  }).done(result => {
    console.log(result);
    if (result === 'success') {
      location.reload();
    } else {
      $('#confirmation button').prop('disabled', false);
      alert('An error occurred');
    }
  }).fail(error => {
    console.log(error);
    alert('An error occurred');
  });
});

$('#override form').submit(e => {
  e.preventDefault();
  $('#override button').prop('disabled', true);
  let additional_reading = $('#additional_reading').val().trim();
  $.post('/add_override', {
    report_id: $('#override').attr('data-id'),
    question: $('#question').val().trim(),
    translation: $('#translation').val().trim(),
    reading: $('#reading').val().trim(),
    additional_reading: additional_reading.length ? additional_reading : undefined,
  }).done(result => {
    console.log(result);
    if (result === 'success') {
      location.reload();
    } else {
      alert(result);
      $('#override button').prop('disabled', false);
    }
  }).fail(error => {
    console.log(error);
    alert('An error occurred');
    $('#override button').prop('disabled', false);
  });
});

$('#edit_override form').submit(e => {
  e.preventDefault();
  $('#edit_override button').prop('disabled', true);
  let override_id = $('#edit_override').attr('data-id');
  $.post('/edit_override', {
    override_id: override_id,
    value: $('#value').val().trim(),
    primary_value: $('#primary').is(':checked'),
  }).done(result => {
    console.log(result);
    if (result === 'success') {
      location.reload();
    } else {
      alert('An error occurred');
      $('#edit_override button').prop('disabled', false);
    }
  }).fail(error => {
    console.log(error);
    alert('An error occurred');
    $('#edit_override button').prop('disabled', false);
  });
});
