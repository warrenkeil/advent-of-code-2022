namespace Day1;

public class DataProcessor
{

  public IEnumerable<string?> ProcessDataFile(string fileName)
  {

    var baseDirectory = System.AppDomain.CurrentDomain.BaseDirectory;
    var filePath = System.IO.Path.Combine(baseDirectory + @"../../../../../../data/", fileName);
    
    Console.WriteLine(filePath);
    var data = File.ReadLines(filePath);
    return data;
  }
}