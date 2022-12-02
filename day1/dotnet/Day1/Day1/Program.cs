
using Day1;

var dataProcessor = new DataProcessor();

var elfData = dataProcessor.ProcessDataFile("data.txt");
Console.WriteLine($"The max number of calories is {CalorieCounter.CountElfCalories(elfData)}");
public static class CalorieCounter
{
  public static int CountElfCalories(IEnumerable<string?> elfFoodString)
  {
    var elfCalorieList = new List<int>();
    var sum = 0;
    foreach (var value in elfFoodString)
    {
      if (value != "")
      {
        var intValue = Int32.Parse(value!);
        sum += intValue;
      }
      if (value == "")
      {
        elfCalorieList.Add(sum);
        sum = 0;
      }
    }
    return elfCalorieList.Max();
  }
  
}