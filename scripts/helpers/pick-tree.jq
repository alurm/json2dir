def pick_tree(tree):
if type == "object" then
  with_entries(
    select(tree[.key]) | .key as $k | .value |= pick_tree(tree[$k])
  )
else .
end;
