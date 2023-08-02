public int SumOfMltpls(int limit) 
{
  return Enumerable.Range(0, limit).Where(num => num % 3 == 0 || num % 5 == 0).Sum(); 
}
