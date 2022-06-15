### Home task

This pallet include 3 callable

- create_club (club_index)
  Take club index and set `sender` as owner of that club index

- add_member (member, club_index)
  Check if club, member exist
  Check if sender is club owner
  Add member to that club index

- remove_member (member, club_index)
  Check if club, member exist
  Check if sender is club owner
  Remove that member
