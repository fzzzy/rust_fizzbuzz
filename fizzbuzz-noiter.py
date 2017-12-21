
for i in range(1, 100):
  fizz = ""
  buzz = ""
  if i % 3 == 0:
    fizz = "fizz"
  if i % 5 == 0:
    buzz = "buzz"
  if fizz == "" and buzz == "":
    print i
  else:
    print "{}{}".format(fizz, buzz)

